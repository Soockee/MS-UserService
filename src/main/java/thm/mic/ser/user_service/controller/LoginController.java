package thm.mic.ser.user_service.controller;

import org.springframework.http.ResponseEntity;
import org.springframework.security.core.context.SecurityContextHolder;
import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import thm.mic.ser.user_service.dto.AppUser;
import thm.mic.ser.user_service.dto.LoginCredentials;
import thm.mic.ser.user_service.dto.Passport;
import thm.mic.ser.user_service.service.AppUserService;

import java.util.Optional;

@Controller
public class LoginController {

    private final AppUserService userService;

    public LoginController(AppUserService userService) {
        this.userService = userService;
    }

    @PostMapping("/login")
    ResponseEntity<Passport> login(@RequestBody LoginCredentials credentials) {

        Optional<AppUser> user = this.userService.getAppUserByEmail(credentials.getEmail());

        if(!user.isPresent()) {
            throw new RuntimeException("No user was found for this Email address");
        }

        String principal = SecurityContextHolder.getContext().getAuthentication().getPrincipal().toString();


        String guid = user.get().getGuid();

        return ResponseEntity.ok(new Passport(guid, principal));
    }
}
