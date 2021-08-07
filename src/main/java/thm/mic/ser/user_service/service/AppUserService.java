package thm.mic.ser.user_service.service;

import org.springframework.amqp.rabbit.core.RabbitTemplate;
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
    private final RabbitTemplate rabbitTemplate;
    private final BCryptPasswordEncoder passwordEncoder;

    static final String topicExchangeName = "spring-boot-exchange";

    static final String queueName = "spring-boot";

    @Autowired
    public AppUserService(AppUserRepository userRepository, RabbitTemplate rabbitTemplate, BCryptPasswordEncoder passwordEncoder) {
        this.userRepository = userRepository;
        this.rabbitTemplate = rabbitTemplate;
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

            informQueueSubscriber();
            return true;
        }
        return false;
    }

    private void informQueueSubscriber() {
        String message = "Deleted";
        this.rabbitTemplate.convertAndSend(topicExchangeName, message);
    }

    public AppUser registerUser(UserRegistrationRequest registrationRequest) {
        AppUser user = new AppUser();
        user.setUsername(registrationRequest.getUsername());
        user.setEmail(registrationRequest.getEmail());
        user.setPassword(passwordEncoder.encode(registrationRequest.getPassword()));
        user.setRole("USER");

        return this.userRepository.save(user);
    }

    public AppUser createUser(String username, String email) {
        AppUser user = new AppUser();
        user.setUsername(username);
        user.setEmail(email);
        return this.userRepository.save(user);
    }

    public Optional<AppUser> getAppUserByEmail(String email) {
        return this.userRepository.getByEmail(email);
    }

    public Optional<AppUser> getAppUserByUsername(String username) {
        return this.userRepository.getByUsername(username);
    }
}
