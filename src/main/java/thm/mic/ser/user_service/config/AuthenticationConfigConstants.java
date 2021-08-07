package thm.mic.ser.user_service.config;

public class AuthenticationConfigConstants {
    public static final String SECRET = "WeWillNotAllowYouToBringYourPetArmadilloAlong";
    public static final long EXPIRATION_TIME = 864000000; // 10 days
    public static final String TOKEN_PREFIX = "Bearer ";
    public static final String HEADER_STRING = "Authorization";
    public static final String SIGN_UP_URL = "/user/register";
    public static final String LOGIN_URL = "/login";
    public static final String ELEVATE_USER = "/elevate/{uuid}";
}