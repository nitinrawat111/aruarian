import { MantineProvider, Text } from "@mantine/core";
import "@mantine/core/styles.css";
import "@mantine/spotlight/styles.css";

export function App() {
  return (
    <MantineProvider>
      <Text>Hello</Text>
    </MantineProvider>
  );
}
