package thm.mic.ser.user_service.service;

import org.springframework.amqp.core.Queue;
import org.springframework.amqp.rabbit.core.RabbitTemplate;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.beans.factory.annotation.Qualifier;
import org.springframework.stereotype.Service;
import thm.mic.ser.user_service.dto.UserCreatedMessage;
import thm.mic.ser.user_service.dto.UserDeletedMessage;

import java.util.UUID;

@Service
public class MessagingService {

    private final RabbitTemplate rabbitTemplate;
    private final Queue userCreationQueue;
    private final Queue userDeletionQueue;

    @Autowired
    public MessagingService(RabbitTemplate rabbitTemplate,
                            @Qualifier("userCreatedUser") Queue userCreationQueue,
                            @Qualifier("userDeletedQueue") Queue userDeletionQueue
    ) {
        this.rabbitTemplate = rabbitTemplate;
        this.userCreationQueue = userCreationQueue;
        this.userDeletionQueue = userDeletionQueue;
    }

    public void sendCreationMessage(String uuid, String name){
        UserCreatedMessage ucm = new UserCreatedMessage();
        ucm.setUuid(UUID.fromString(uuid));
        ucm.setName(name);
        this.rabbitTemplate.convertAndSend(userCreationQueue.getName(), ucm);
    }

    public void sendDeletionMessage(String message){
        UserDeletedMessage udm = new UserDeletedMessage();
        udm.setUuid(UUID.fromString(message));
        this.rabbitTemplate.convertAndSend(userDeletionQueue.getName(), udm);
    }
}
