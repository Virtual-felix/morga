import React, { useState } from "react";
import axios from "axios";
import { Container, Tabs, Tab, Box, Fab } from "@material-ui/core";
import { Add as AddIcon } from "@material-ui/icons";
import { ThemeProvider } from "@material-ui/styles";
import "typeface-roboto";

import { getCustomTheme } from "./theme";
import Navigation from "./component/navigation";
import List from "component/list";

function App() {
  const [darkMode, setDarkMode] = useState(true);
  const theme = getCustomTheme(darkMode);

  const switchDarkMode = (mode) => {
    setDarkMode(mode);
  };

  axios
    .get(process.env.REACT_APP_API_URL)
    .then((res) => {
      console.log(res);
    })
    .catch((err) => {
      console.log(err);
    });

  return (
    <ThemeProvider theme={theme}>
      <Box
        fontFamily="roboto"
        minHeight="100vh"
        bgcolor={theme.palette.background.default}
      >
        <Navigation darkMode={darkMode} switchDarkMode={switchDarkMode} />
        <Container>
          <Box
            position="relative"
            top={theme.spacing(10)}
            bgcolor={theme.palette.background.paper}
            minHeight={theme.spacing(62)}
            minWidth="100%"
          >
            {/* Tabs */}
            <Box bgcolor={theme.palette.background.paper}>
              <Tabs
                value={0}
                indicatorColor="primary"
                textColor="primary"
                centered
              >
                <Tab label="Item One" />
                <Tab label="Item Two" />
                <Tab label="Item Three" />
              </Tabs>
            </Box>

            {/* List (aka tab content) */}
            <List />

            {/* Add item */}
            <Box
              position="absolute"
              bottom={theme.spacing(2)}
              right={theme.spacing(2)}
            >
              <Fab color="primary" aria-label="add">
                <AddIcon />
              </Fab>
            </Box>
          </Box>
        </Container>
      </Box>
    </ThemeProvider>
  );
}

export default App;
