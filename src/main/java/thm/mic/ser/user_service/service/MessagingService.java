package thm.mic.ser.user_service.service;

import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.ObjectMapper;
import lombok.extern.log4j.Log4j2;
import org.springframework.amqp.core.Message;
import org.springframework.amqp.core.MessageBuilder;
import org.springframework.amqp.core.MessageProperties;
import org.springframework.amqp.core.Queue;
import org.springframework.amqp.rabbit.core.RabbitTemplate;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.beans.factory.annotation.Qualifier;
import org.springframework.stereotype.Service;
import thm.mic.ser.user_service.dto.UserCreatedMessage;
import thm.mic.ser.user_service.dto.UserDeletedMessage;

import java.util.UUID;

@Log4j2
@Service
public class MessagingService {

    private final RabbitTemplate rabbitTemplate;
    private final Queue userCreationQueue;
    private final Queue userDeletionQueue;
    private final ObjectMapper objectMapper;

    @Autowired
    public MessagingService(RabbitTemplate rabbitTemplate,
                            @Qualifier("userCreatedUser") Queue userCreationQueue,
                            @Qualifier("userDeletedQueue") Queue userDeletionQueue,
                            ObjectMapper objectMapper) {
        this.rabbitTemplate = rabbitTemplate;
        this.userCreationQueue = userCreationQueue;
        this.userDeletionQueue = userDeletionQueue;
        this.objectMapper = objectMapper;
    }

    public void sendCreationMessage(String uuid, String name){
        UserCreatedMessage ucm = new UserCreatedMessage();
        ucm.setUuid(UUID.fromString(uuid));
        ucm.setName(name);

        this.rabbitTemplate.convertAndSend(userCreationQueue.getName(), createMessageFromObject(ucm));
    }

    public void sendDeletionMessage(String message){
        UserDeletedMessage udm = new UserDeletedMessage();
        udm.setUuid(UUID.fromString(message));
        this.rabbitTemplate.convertAndSend(userDeletionQueue.getName(), createMessageFromObject(udm));
    }

    private Message createMessageFromObject(Object object) {
        String orderJson = null;
        try {
            orderJson = objectMapper.writeValueAsString(object);
        } catch (JsonProcessingException e) {
            throw new RuntimeException("Could not map to json : " + e);
        }
        return MessageBuilder
                .withBody(orderJson.getBytes())
                .setContentType(MessageProperties.CONTENT_TYPE_JSON)
                .build();
    }
}
