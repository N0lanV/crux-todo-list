import "@mantine/core/styles.css";
import { MantineProvider } from "@mantine/core";
import { theme } from "./theme";
import { greet } from 'wasm';

export default function App() {
  greet("Hello world");
  return <MantineProvider theme={theme}>App</MantineProvider>;
}
