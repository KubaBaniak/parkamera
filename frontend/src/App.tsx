import Navbar from "./components/navbar/navbar"
import SlotsInfoList from "./components/slots-info/slots-info"
import Stack from '@mui/material/Stack';
import { Container, ThemeProvider, Typography } from "@mui/material";
import History from "./components/history/history";
import { createTheme, responsiveFontSizes } from '@mui/material/styles';

let theme = createTheme();
theme = responsiveFontSizes(theme);

function App() {
  return (
    <>
      <Navbar />
      <Stack
        sx={{ m: 4 }}
        direction="row"
        justifyContent="space-around"
        alignItems="center"
        spacing={2}
      >
        <Container maxWidth="sm">
          <Stack
            justifyContent="space-around"
            alignItems="center"
            spacing={2}
          >
            <ThemeProvider theme={theme}>
              <Typography variant="h3">Current parking state</Typography>
            </ThemeProvider>
            <SlotsInfoList />
          </Stack >
        </Container>

        <Container maxWidth="sm">
          <Stack
            justifyContent="space-around"
            alignItems="center"
            spacing={2}
          >
            <ThemeProvider theme={theme}>
              <Typography variant="h3">History</Typography>
            </ThemeProvider>
            <History />
          </Stack >
        </Container>

      </Stack >
    </>
  )
}

export default App
