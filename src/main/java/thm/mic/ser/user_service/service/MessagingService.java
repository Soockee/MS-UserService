package thm.mic.ser.user_service.service;

import org.springframework.amqp.core.Queue;
import org.springframework.amqp.rabbit.core.RabbitTemplate;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;
import thm.mic.ser.user_service.dto.UserDeletedMessage;

import java.util.UUID;

@Service
public class MessagingService {

    private final RabbitTemplate rabbitTemplate;
    private final Queue queue;

    @Autowired
    public MessagingService(RabbitTemplate rabbitTemplate, Queue queue) {
        this.rabbitTemplate = rabbitTemplate;
        this.queue = queue;
    }

    public void sendMessage(String message){
        UserDeletedMessage udm = new UserDeletedMessage();
        udm.setUuid(UUID.fromString(message));
        this.rabbitTemplate.convertAndSend(queue.getName(), udm);
    }
}
