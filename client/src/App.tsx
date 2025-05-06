import { ThemeProvider, createTheme } from "@mui/material/styles";
import Container from "@mui/material/Container";
import UrlShortener from "./UrlShortener";

const theme = createTheme({
  palette: {
    primary: {
      main: "#1976d2",
    },
  },
});

const App: React.FC = () => {
  return (
    <ThemeProvider theme={theme}>
      <Container maxWidth="sm">
        <UrlShortener />
      </Container>
    </ThemeProvider>
  );
};

export default App;
