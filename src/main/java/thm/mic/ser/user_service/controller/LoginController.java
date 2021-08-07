package thm.mic.ser.user_service.controller;

import org.springframework.http.ResponseEntity;
import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import thm.mic.ser.user_service.dto.LoginCredentials;
import thm.mic.ser.user_service.dto.Passport;

@Controller
public class LoginController {

    @PostMapping("/login")
    ResponseEntity<Passport> login(@RequestBody LoginCredentials credentials) {
        return ResponseEntity.ok(new Passport("", ""));
    }
}
