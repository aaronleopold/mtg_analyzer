package com.mtg_analyzer.backend.exceptions;

public class UnauthenticatedException extends RuntimeException{
    public UnauthenticatedException(String message) {
        super(message);
    }
}
