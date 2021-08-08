package thm.mic.ser.user_service.config;

import org.springframework.amqp.core.*;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;

@Configuration
public class RabbitConfiguration {

    @Value("${rabbit.exchange}")
    static final String directExchange = "direct-exchange";

    @Bean
    Binding userDeletedBinding(Queue queue, DirectExchange exchange) {
        return BindingBuilder.bind(queue).to(exchange).with("project.user.deleted");
    }

    @Bean
    Queue userDeletedQueue() {
        return new Queue("userDeletedQueue", false);
    }

    @Bean
    public DirectExchange exchange() {
        return new DirectExchange(directExchange, false, false);
    }
}
