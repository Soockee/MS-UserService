package thm.mic.ser.user_service.dto;

import com.fasterxml.jackson.annotation.JsonIgnore;
import com.sun.istack.NotNull;
import lombok.Data;

import javax.persistence.*;
import java.util.UUID;

@Data
@Entity // User is a reserved word in postgres
public class AppUser {

    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    Integer id;

    @Column
    @NotNull
    String guid = UUID.randomUUID().toString();

    @Column
    @NotNull
    String username;

    @Column(unique = true)
    @NotNull
    String email;

    @Column
    @JsonIgnore
    private String password;

    @Column
    @NotNull
    private String role;
}
