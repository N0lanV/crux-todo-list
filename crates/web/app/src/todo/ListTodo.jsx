import {useWasm, View} from "../useWasm.js";
import {ActionIcon, Badge, Divider, Group, List, Text} from "@mantine/core";

export default function ListTodo() {
        const update = View.Todo.update;
        const todo = useWasm(View.Todo);

        if (todo.task_list.length == 0){
                return []
        }

        let priorityDisplay = (priority) => {
                switch (priority){
                        case 'Low': return "📅 low";
                        case 'Medium': return "⏳ medium";
                        case 'High': return "💣 high";
                }
        };

        return [
                <Divider key="ListTodoDivider" my="md" mb="xl" />,
                <List
                        key="ListTodoList"
                        ml="md"
                        mr="md"
                        mb="sm"
                        spacing="md"
                        styles={{
                                itemWrapper: {width: "100%"},
                                itemLabel: {flexGrow: 1}
                        }}
                >
                        {
                                todo.task_list.map(task =>
                                        <List.Item
                                                icon={
                                                        <Badge color="orange">
                                                                {priorityDisplay(task.priority)}
                                                        </Badge>
                                                }
                                                key={"Task item "+task.id}
                                        >
                                                <Group justify="space-between" wrap="nowrap">
                                                        <Text lineClamp={1} inline>
                                                                {task.title}
                                                        </Text>
                                                        <Group justify="space-between" wrap="nowrap">
                                                                <ActionIcon
                                                                        me="auto"
                                                                        variant="light"
                                                                        color="orange"
                                                                        aria-label="Edit"
                                                                        onClick={() => update({EditTask: task.id})}>
                                                                        ✏️
                                                                </ActionIcon>
                                                                <ActionIcon
                                                                        me="auto"
                                                                        variant="light"
                                                                        color="orange"
                                                                        aria-label="Delete"
                                                                        onClick={() => update({RemoveTask: task.id})}>
                                                                        🗑
                                                                </ActionIcon>
                                                        </Group>
                                                </Group>
                                        </List.Item>
                                )
                        }
                </List>
        ];
}