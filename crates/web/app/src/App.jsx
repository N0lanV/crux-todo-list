import "@mantine/core/styles.css";
import { MantineProvider } from "@mantine/core";
import { theme } from "./theme";
import Todo from "./Todo.jsx";

export default function App() {
        return (
                <MantineProvider theme={theme}>
                        <Todo />
                </MantineProvider>
        );
}
