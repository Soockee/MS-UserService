package thm.mic.ser.user_service.dto;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.databind.annotation.JsonSerialize;
import lombok.Data;

import java.io.Serializable;
import java.util.UUID;

@Data
@JsonSerialize
public class UserDeletedMessage implements Serializable {
    @JsonProperty
    UUID uuid;
}
