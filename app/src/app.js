import React from 'react';
import Box from '@material-ui/core/Box';
import logo from './logo512white.png';
import { ThemeProvider } from "@material-ui/styles";
import { theme } from './theme';
import "typeface-roboto";

function App() {
  return (
    <ThemeProvider theme={theme}>
      <Box bgcolor="#f5f7f8" minHeight="100vh" textAlign="center" fontFamily="roboto">
        <Box>
          <img src={logo} alt="logo" />
        </Box>
        <Box>
          Hello Phoenix
      </Box>
      </Box>
    </ThemeProvider>
  );
}

export default App;
