import { MantineProvider } from "@mantine/core";
import { SpotlightActionData, Spotlight } from "@mantine/spotlight";
import {
  IconHome,
  IconDashboard,
  IconFileText,
  IconSearch,
} from "@tabler/icons-react";
import "@mantine/core/styles.css";
import "@mantine/spotlight/styles.css";
import "./global.css";

// Dummy actions for now
// TODO: Replace with real actions
const actions: SpotlightActionData[] = [
  {
    id: "home",
    label: "Home",
    description: "Get to home page",
    onClick: () => console.log("Home"),
    leftSection: <IconHome size={24} stroke={1.5} />,
  },
  {
    id: "dashboard",
    label: "Dashboard",
    description: "Get full information about current system status",
    onClick: () => console.log("Dashboard"),
    leftSection: <IconDashboard size={24} stroke={1.5} />,
  },
  {
    id: "documentation",
    label: "Documentation",
    description: "Visit documentation to lean more about all features",
    onClick: () => console.log("Documentation"),
    leftSection: <IconFileText size={24} stroke={1.5} />,
  },
];

export function App() {
  return (
    <MantineProvider defaultColorScheme="dark">
      <Spotlight
        opacity={0.6}
        actions={actions}
        nothingFound="Confused oonga boonga."
        searchProps={{
          leftSection: <IconSearch size={20} stroke={1.5} />,
          placeholder: "Search...",
        }}
        highlightQuery={true}
        // Make the spotlight in the current DOM tree instead of a portal
        // Can be used if really needed. But not for now
        withinPortal={false}
        // Make the spotlight modal take the full screen (otherwise it takes a portion of the parent element)
        fullScreen={true}
        // We want this to be opened always,
        // only way to close this is close the overlay window
        forceOpened={true}
        // The overlay window is centered and the spotlight is the only component in it
        // So this doesn't make much sense. but let's do it anyway
        centered={true}
        styles={{
          content: {
            // Make the spotlight take the full height
            // Otherwise, the content is displayed only on a part of <Spotlight.Root>
            height: "100vh",
          },
        }}
      />
    </MantineProvider>
  );
}
