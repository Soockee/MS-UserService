package thm.mic.ser.user_service.service;

import org.springframework.security.crypto.bcrypt.BCryptPasswordEncoder;
import thm.mic.ser.user_service.dto.AppUser;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;
import thm.mic.ser.user_service.dto.UserRegistrationRequest;
import thm.mic.ser.user_service.repository.AppUserRepository;

import java.util.List;
import java.util.Optional;

@Service
public class AppUserService {
    private final AppUserRepository userRepository;
    private final MessagingService messagingService;
    private final BCryptPasswordEncoder passwordEncoder;

    @Autowired
    public AppUserService(AppUserRepository userRepository, MessagingService messagingService, BCryptPasswordEncoder passwordEncoder) {
        this.userRepository = userRepository;
        this.messagingService = messagingService;
        this.passwordEncoder = passwordEncoder;
    }

    public List<AppUser> getAllAppUsers(){
        return this.userRepository.findAll();
    }

    public AppUser getAppUserByUuid(String uuid){
        Optional<AppUser> user = this.userRepository.getByUserGUID(uuid);
        if (user.isPresent()) {
            return user.get();
        } else {
            throw new IllegalArgumentException();
        }
    }

    public AppUser updateAppUser(AppUser user){
        return this.userRepository.save(user);
    }

    public boolean deleteAppUser(String uuid){
        Optional<AppUser> user = this.userRepository.getByUserGUID(uuid);
        if(user.isPresent()) {
            this.userRepository.delete(user.get());

            informDeletionQueueSubscriber(uuid);
            return true;
        }
        return false;
    }

    private void informDeletionQueueSubscriber(String uuid) {
        this.messagingService.sendDeletionMessage(uuid);
    }

    public AppUser registerUser(UserRegistrationRequest registrationRequest) {
        AppUser user = new AppUser();
        user.setUsername(registrationRequest.getUsername());
        user.setEmail(registrationRequest.getEmail());
        user.setPassword(passwordEncoder.encode(registrationRequest.getPassword()));
        user.setRole("USER");

        informUserCreationQueueSubscriber(user.getGuid(), user.getUsername());

        return this.userRepository.save(user);
    }

    private void informUserCreationQueueSubscriber(String guid, String username) {
        this.messagingService.sendCreationMessage(guid, username);
    }

    public AppUser createUser(String username, String email) {
        AppUser user = new AppUser();
        user.setUsername(username);
        user.setEmail(email);
        informUserCreationQueueSubscriber(user.getGuid(), user.getUsername());

        return this.userRepository.save(user);
    }

    public Optional<AppUser> getAppUserByEmail(String email) {
        return this.userRepository.getByEmail(email);
    }

    public Optional<AppUser> getAppUserByUsername(String username) {
        return this.userRepository.getByUsername(username);
    }
}
