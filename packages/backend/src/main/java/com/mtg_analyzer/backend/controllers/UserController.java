package com.mtg_analyzer.backend.controllers;

import com.mtg_analyzer.backend.entities.User;
import com.mtg_analyzer.backend.exceptions.BadRequestException;
import com.mtg_analyzer.backend.exceptions.UnauthenticatedException;
import com.mtg_analyzer.backend.exceptions.UserAuthenticationException;
import com.mtg_analyzer.backend.repositories.UserRepository;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.context.annotation.Bean;
import org.springframework.dao.DataIntegrityViolationException;
import org.springframework.http.ResponseEntity;
import org.springframework.security.crypto.password.PasswordEncoder;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;
import org.springframework.security.crypto.bcrypt.BCryptPasswordEncoder;

import javax.persistence.EntityExistsException;
import javax.servlet.http.HttpServletRequest;
import java.io.Serializable;
import java.util.Optional;


// TODO: look into *secure* authentication using Spring. Consider splitting up Authentication to separate controller.

@RestController
@RequestMapping("/user")
public class UserController {

    private final UserRepository userRepository;
    private final PasswordEncoder passwordEncoder = passwordEncoder();

    @Bean
    public PasswordEncoder passwordEncoder() {
        return new BCryptPasswordEncoder();
    }

    // TODO: look into what Autowired really does
    @Autowired
    public UserController(UserRepository userRepository) {
        this.userRepository = userRepository;
    }

    @PostMapping("/login")
    public ResponseEntity<User> login(@RequestBody UserLoginFragment body) {
        String username = body.username;
        String email = body.email;
        String password = body.password;

        if (username == null && email == null) {
            throw new BadRequestException("You must provide either an email or username to login.");
        }

        Optional<User> user;

        if (username != null) {
            user = userRepository.findByUsername(username);
        } else {
            user = userRepository.findByEmail(email);
        }

        // IDE did not like isEmpty here
        if (!user.isPresent() || !this.passwordEncoder.matches(password, user.get().getPassword())) {
            // I am not going to tell users if the account exists or if they used wrong creds
            throw new UserAuthenticationException("Unable to login using those credentials.");
        } else {
            // TODO: I do NOT like returning the password, I need to research more about auth
//            request.getSession().setAttribute("VIEWER", user.getId());
            return ResponseEntity.ok(user.get());
        }
    }

    @PostMapping("/register")
    public ResponseEntity<User> register(@RequestBody UserRegisterFragment body) {
        String username = body.username;
        String email = body.email;
        String firstName = body.firstName;
        String lastName = body.lastName;
        String password = body.password;

        // TODO: check null params

        try {
            // password should get hashed in User constructor
            User newUser = new User(
                    username,
                    email,
                    firstName,
                    lastName,
                    password
            );

            userRepository.saveAndFlush(newUser);

            return ResponseEntity.ok(newUser);
        } catch (EntityExistsException | DataIntegrityViolationException e) {
            // I am not going to tell users if the account exists
            // TODO: determine what the catch structure should look like for this. Ideally I want to catch small things
            // on the client (like malformed email or whatever)
            throw new UserAuthenticationException("Something went wrong during the registration process.");
        }

    }

    @PostMapping("/viewer")
    public ResponseEntity<User> viewer(HttpServletRequest request) {
            try {
                User viewer = userRepository.findById(
                        (Long) request.getSession().getAttribute("VIEWER")
                ).orElse(null);

                if (viewer != null) {
                    return ResponseEntity.ok(viewer);
                } else {
                    throw new UnauthenticatedException("No session found");
                }
            } catch (Exception e) {
                throw new UnauthenticatedException("No session found");
            }

    }

    public static class UserLoginFragment implements Serializable { public String username; public String email; public String password; }

    public static class UserRegisterFragment implements Serializable {
        public String username;
        public String email;
        public String firstName;
        public String lastName;
        public String password;
    }
}
