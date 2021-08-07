package thm.mic.ser.user_service.dto;

import com.sun.istack.NotNull;
import lombok.AllArgsConstructor;
import lombok.Data;
import lombok.NoArgsConstructor;

@Data
@NoArgsConstructor
@AllArgsConstructor
public class LoginCredentials
{
    @NotNull()
    private String username;

    @NotNull()
    private String password;
}
