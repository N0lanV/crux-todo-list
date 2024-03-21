import {
        ActionIcon,
        Badge,
        Button,
        Card,
        Center, Container,
        Divider,
        Group,
        Image,
        Input,
        List,
        SegmentedControl,
        Text
} from "@mantine/core";
import img from "./logo-banner-001.png";
import {useWasm, View} from "./useWasm.js";

export default function Todo() {
        const update = View.Todo.update;
        const todo = useWasm(View.Todo);

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
                                <SegmentedControl
                                        fullWidth
                                        withItemsBorders={false}
                                        data={['📅 low', '⏳ medium', '💣 high']}
                                        color="orange"
                                        mb="sm"
                                />
                                <Input
                                        me="md"
                                        ms="md"
                                        placeholder="Task name"
                                        value={todo.task_new_title}
                                        onChange={(event) => update({UpdateTaskNewTitle: event.currentTarget.value})}
                                />
                                <Center me="xl" ms="xl">
                                        <Button
                                                mt="lg"
                                                mb="sm"
                                                color="orange"
                                                fullWidth
                                                radius="md"
                                        >
                                                💾 Add a task
                                        </Button>
                                </Center>
                                <Divider my="md" mb="xl" />
                                <List
                                        ml="md"
                                        mr="md"
                                        mb="sm"
                                        spacing="md"
                                        styles={{
                                                itemWrapper: {width: "100%"},
                                                itemLabel: {flexGrow: 1}
                                        }}
                                >
                                        <List.Item
                                                icon={
                                                        <Badge color="orange">📅 low</Badge>
                                                }
                                        >
                                                <Group justify="space-between" wrap="nowrap">
                                                        <Text lineClamp={1} inline>
                                                                Wash dishes
                                                        </Text>
                                                        <Group justify="space-between" wrap="nowrap">
                                                                <ActionIcon me="auto" variant="light" color="orange" aria-label="Edit">
                                                                        ✏️
                                                                </ActionIcon>
                                                                <ActionIcon me="auto" variant="light" color="orange" aria-label="Delete">
                                                                        🗑
                                                                </ActionIcon>
                                                        </Group>
                                                </Group>
                                        </List.Item>
                                        <List.Item
                                                icon={
                                                        <Badge color="orange">💣 high</Badge>
                                                }
                                        >
                                                <Group justify="space-between" wrap="nowrap">
                                                        <Text lineClamp={1} inline>
                                                                Pay bills
                                                        </Text>
                                                        <Group justify="space-between" wrap="nowrap">
                                                                <ActionIcon me="auto" variant="light" color="orange" aria-label="Edit">
                                                                        ✏️
                                                                </ActionIcon>
                                                                <ActionIcon me="auto" variant="light" color="orange" aria-label="Delete">
                                                                        🗑
                                                                </ActionIcon>
                                                        </Group>
                                                </Group>
                                        </List.Item>
                                </List>
                        </Card>
                </Container>
        )
}