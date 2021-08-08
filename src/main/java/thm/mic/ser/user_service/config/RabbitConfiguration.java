package thm.mic.ser.user_service.config;

import org.springframework.amqp.core.*;
import org.springframework.beans.factory.annotation.Qualifier;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;

@Configuration
public class RabbitConfiguration {

    @Value("${rabbit.exchange.direct}")
    static final String directExchange = "direct-exchange";

    @Value("${rabbit.exchange.news}")
    static final String newsExchange = "news";

    @Bean
    Binding userCreatedBinding(@Qualifier("userCreatedUser") Queue queue, TopicExchange exchange) {
        return BindingBuilder.bind(queue).to(exchange).with("news.user.create");
    }

    @Bean
    Binding userDeletedBinding(@Qualifier("userDeletedQueue") Queue queue, DirectExchange exchange) {
        return BindingBuilder.bind(queue).to(exchange).with("project.user.deleted");
    }

    @Bean
    Binding userAddQueue(@Qualifier("userAddQueue") Queue queue, DirectExchange exchange) {
        return BindingBuilder.bind(queue).to(exchange).with("project.user.add");
    }

    @Bean
    Queue userAddQueue() {
        return new Queue("userAddQueue");
    }

    @Bean
    Queue userDeletedQueue() {
        return new Queue("userDeletedQueue");
    }

    @Bean
    Queue userCreatedUser() {
        return new Queue("userCreated");
    }

    @Bean
    public TopicExchange topicExchange() {
        return new TopicExchange(newsExchange);
    }

    @Bean
    public DirectExchange directExchange() {
        return new DirectExchange(directExchange);
    }
}
