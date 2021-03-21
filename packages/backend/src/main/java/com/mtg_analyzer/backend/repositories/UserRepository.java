package com.mtg_analyzer.backend.repositories;

import com.mtg_analyzer.backend.entities.User;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.data.jpa.repository.Query;
import org.springframework.data.repository.query.Param;

public interface UserRepository extends JpaRepository<User, Long> {

    @Query("SELECT u FROM User u where u.email = :email")
    User findByEmail(@Param("email") String email);

    @Query("SELECT u FROM User u where u.username = :username")
    User findByUsername(@Param("username") String username);

}
