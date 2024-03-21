import {
        Card,
        Container,
        Image,
} from "@mantine/core";
import img from "./logo-banner-001.png";
import AddTodo from "./todo/AddTodo.jsx";
import ListTodo from "./todo/ListTodo.jsx";

export default function Todo() {

        return (
                <Container size={500} mt="lg">
                        <Card shadow="sm" padding="none" radius="md" withBorder>
                                <Card.Section>
                                        <Image
                                                src={img}
                                                height={200}
                                                fit="cover"
                                                alt="noobi.fi logo"
                                        />
                                </Card.Section>
                                <AddTodo />
                                <ListTodo />
                        </Card>
                </Container>
        )
}