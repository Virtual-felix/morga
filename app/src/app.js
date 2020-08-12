import React from 'react';
import axios from 'axios';
import Box from '@material-ui/core/Box';
import logo from './logo512white.png';
import { ThemeProvider } from "@material-ui/styles";
import { theme } from './theme';
import "typeface-roboto";

function App() {
  axios.get(process.env.REACT_APP_API_URL)
    .then(res => {
      console.log(res);
    })
    .catch(err => {
      console.log(err);
    });

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
