package thm.mic.ser.user_service.service;

import org.springframework.amqp.core.Binding;
import org.springframework.amqp.core.BindingBuilder;
import org.springframework.amqp.core.Queue;
import org.springframework.amqp.core.TopicExchange;
import org.springframework.amqp.rabbit.core.RabbitTemplate;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.context.annotation.Bean;
import org.springframework.stereotype.Component;
import org.springframework.stereotype.Service;

@Service
public class MessagingService {

    @Value("${rabbit.exchange}")
    static final String topicExchangeName = "project.user.deleted";

    private final RabbitTemplate rabbitTemplate;

    @Autowired
    public MessagingService(RabbitTemplate rabbitTemplate) {
        this.rabbitTemplate = rabbitTemplate;
    }

    @Bean
    Binding userDeletedBinding(Queue queue, TopicExchange exchange) {
        return BindingBuilder.bind(queue).to(exchange).with("project.user.deleted");
    }

    @Bean
    Queue userDeletedQueue() {
        return new Queue("userDeletedQueue", false);
    }

    @Bean
    TopicExchange exchange() {
        return new TopicExchange(topicExchangeName);
    }

    public void sendMessage(String message){
        this.rabbitTemplate.convertAndSend("project.user.deleted", message);
    }
}
