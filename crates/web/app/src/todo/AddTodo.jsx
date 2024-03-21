import {useWasm, View} from "../useWasm.js";
import {ActionIcon, Button, Center, Group, Input, SegmentedControl} from "@mantine/core";

export default function AddTodo(){
        const update = View.Todo.update;
        const todo = useWasm(View.Todo);

        return [
                <SegmentedControl
                        key="AddTodoSegmentedControl"
                        fullWidth
                        withItemsBorders={false}
                        data={[
                                {label: '📅 low', value: 'Low'},
                                {label: '⏳ medium', value: 'Medium'},
                                {label: '💣 high', value: 'High'}
                        ]}
                        color="orange"
                        mb="sm"
                        value={todo.task_new_priority}
                        onChange={value => update({UpdateTaskNewPriority: value})}
                />,
                <Input
                        key="AddTodoInput"
                        me="md"
                        ms="md"
                        placeholder="Task name"
                        value={todo.task_new_title}
                        onChange={event => update({UpdateTaskNewTitle: event.currentTarget.value})}
                />,
                <Center me="xl" ms="xl" key="AddTodoCenter">
                        {
                                todo.task_new_id
                                        ? <Group
                                                styles={{
                                                        root: {width: "100%"}
                                                }}
                                                justify="space-between"
                                                wrap="nowrap">
                                                <ActionIcon
                                                        mt="lg"
                                                        mb="sm"
                                                        me="auto"
                                                        color="orange"
                                                        aria-label="Edit"
                                                        onClick={() => update('CancelEditTask')}>
                                                        ❌
                                                </ActionIcon>
                                                <ActionIcon
                                                        mt="lg"
                                                        mb="sm"
                                                        me="auto"
                                                        color="orange"
                                                        aria-label="Delete"
                                                        onClick={() => update({RemoveTask: task.id})}>
                                                        🗑
                                                </ActionIcon>
                                                <Button
                                                        mt="lg"
                                                        mb="sm"
                                                        color="orange"
                                                        fullWidth
                                                        radius="md"
                                                        onClick={() => update('UpsertTask')}
                                                >
                                                        💾 Edit task
                                                </Button>
                                        </Group>
                                        : <Button
                                                mt="lg"
                                                mb="sm"
                                                color="orange"
                                                fullWidth
                                                radius="md"
                                                onClick={() => update('UpsertTask')}
                                        >
                                                💾 Add a task
                                        </Button>
                        }
                </Center>
        ];
}