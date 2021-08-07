package thm.mic.ser.user_service.controller;

import thm.mic.ser.user_service.dto.AppUser;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;
import thm.mic.ser.user_service.dto.UserRegistrationRequest;
import thm.mic.ser.user_service.service.AppUserService;

import java.util.List;

@RestController()
@RequestMapping("/user")
public class AppUserController {

    private final AppUserService userService;

    @Autowired
    public AppUserController(AppUserService userService) {
        this.userService = userService;
    }

    @GetMapping("")
    public ResponseEntity<List<AppUser>> getAllAppUsers(){
        return ResponseEntity.ok(this.userService.getAllAppUsers());
    }

    @GetMapping("{uuid}")
    ResponseEntity<AppUser> getAppUser(@PathVariable String uuid){
        return ResponseEntity.ok(this.userService.getAppUserByUuid(uuid));
    }

    @PostMapping("")
    ResponseEntity<AppUser> createAppUser(
            @RequestParam String username,
            @RequestParam String email
    ) {
        return ResponseEntity.ok(this.userService.createUser(username, email));
    }

    @PostMapping("/register")
    public ResponseEntity<AppUser> createUser (@RequestBody UserRegistrationRequest registrationRequest) {
        AppUser user = userService.registerUser(registrationRequest);
        return ResponseEntity.ok().build();
    }

    @PutMapping("{uuid}")
    ResponseEntity<AppUser> updateAppUser(@PathVariable String uuid,
                                    @RequestParam String username,
                                    @RequestParam String email){

        AppUser user = this.userService.getAppUserByUuid(uuid);
        user.setUsername(username);
        user.setEmail(email);
        return ResponseEntity.ok(this.userService.updateAppUser(user));
    }

    @PutMapping("/elevate/{uuid}")
    ResponseEntity<AppUser> setUserRole(@PathVariable String uuid, @RequestParam String role){

        AppUser user = this.userService.getAppUserByUuid(uuid);
        user.setRole(role);
        return ResponseEntity.ok(this.userService.updateAppUser(user));
    }

    @DeleteMapping("{uuid}")
    ResponseEntity<Boolean> deleteAppUser(@PathVariable String uuid){
        return ResponseEntity.ok(this.userService.deleteAppUser(uuid));
    }

}
