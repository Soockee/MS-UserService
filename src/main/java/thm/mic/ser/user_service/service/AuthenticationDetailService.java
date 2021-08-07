package thm.mic.ser.user_service.service;

import org.springframework.security.core.GrantedAuthority;
import org.springframework.security.core.authority.SimpleGrantedAuthority;
import org.springframework.security.core.userdetails.UserDetails;
import org.springframework.security.core.userdetails.UserDetailsService;
import org.springframework.security.core.userdetails.UsernameNotFoundException;
import org.springframework.stereotype.Service;

import java.util.Arrays;
import java.util.Collection;

import lombok.RequiredArgsConstructor;
import thm.mic.ser.user_service.dto.AppUser;

@Service
@RequiredArgsConstructor
public class AuthenticationDetailService implements UserDetailsService {

    private final AppUserService userService;

    @Override public UserDetails loadUserByUsername(String uuid) throws UsernameNotFoundException {
        AppUser appUser = userService.getAppUserByUuid(uuid);
        if (appUser == null) {
            throw new UsernameNotFoundException(uuid);
        }
        return new org.springframework.security.core.userdetails.User(appUser.getUsername(),
                appUser.getPassword(), getAuthorities(appUser.getRole()));
    }

    private Collection<? extends GrantedAuthority> getAuthorities(String role) {
        return Arrays.asList(new SimpleGrantedAuthority(role));
    }
}