package com.mtg_analyzer.backend.controller;

import com.mtg_analyzer.backend.exception.ResourceNotFoundException;
import com.mtg_analyzer.backend.model.JwtRequest;
import com.mtg_analyzer.backend.model.JwtResponse;
import com.mtg_analyzer.backend.model.RegistrationRequest;
import com.mtg_analyzer.backend.service.UserService;
import com.mtg_analyzer.backend.utility.JwtUtility;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.security.authentication.AuthenticationManager;
import org.springframework.security.authentication.BadCredentialsException;
import org.springframework.security.authentication.UsernamePasswordAuthenticationToken;
import org.springframework.security.core.userdetails.UserDetails;
import org.springframework.security.core.userdetails.UsernameNotFoundException;
import org.springframework.web.bind.annotation.*;

import java.util.Optional;

@RestController
@RequestMapping("/user")
public class UserController {

    @Autowired
    private JwtUtility jwtUtility;

    @Autowired
    private AuthenticationManager authenticationManager;

    @Autowired
    private UserService userService;

    @PostMapping("/login")
    public JwtResponse login(@RequestBody JwtRequest request) throws Exception{

        try {
            authenticationManager.authenticate(
                    new UsernamePasswordAuthenticationToken(
                            request.getUsername(),
                            request.getPassword()
                    )
            );
        } catch (BadCredentialsException e) {
            throw new Exception("INVALID_CREDENTIALS", e);
        }

        final UserDetails userDetails
                = userService.loadUserByUsername(request.getUsername());

        final String token =
                jwtUtility.generateToken(userDetails);

        return  new JwtResponse(token);

    }

    @PostMapping("/register")
    public JwtResponse register(@RequestBody RegistrationRequest request) {

        UserDetails newUser = userService.registerNewUser(
                request.getUsername(),
                request.getEmail(),
                request.getFirstName(),
                request.getLastName(),
                request.getPassword()
        );

        final String token =
                jwtUtility.generateToken(newUser);

        return new JwtResponse(token);

    }

    // TODO: make me
    // FIXME: I don't like the way this is structured. I don't think I will keep it this way
    @PostMapping("/profile/{username}")
    public UserDetails getUserProfile(@PathVariable Optional<String> username, @RequestBody Optional<String> token) {

        if (username.isPresent()) {

            try {
                return userService.loadUserByUsername(username.get());
            } catch (UsernameNotFoundException e) {
                throw new ResourceNotFoundException("TODO: make me");
            }

        } else if (token.isPresent()) {
            // load viewer user


        } else {
            // I don't LOVE this tbh
        }

        // TODO:
        return null;

    }
}
