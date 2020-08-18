import React from "react";
import {
  AppBar,
  Toolbar,
  Box,
  Icon,
  FormGroup,
  FormControlLabel,
  Switch,
} from "@material-ui/core";
import Logo from "logo512white.png";

const logoStyle = { width: "auto", height: "50px" };

export default function Navigation({ switchDarkMode, darkMode }) {
  const handleDarkMode = (event) => {
    switchDarkMode(event.target.checked);
  };

  return (
    <AppBar position="relative">
      <Toolbar>
        <Box display="flex" alignItems="center">
          <Icon style={logoStyle}>
            <img style={logoStyle} alt="logo" src={Logo} />
          </Icon>
        </Box>
        <Box pl={2} fontSize={20} fontWeight="fontWeightBold">
          Phoenix
        </Box>
        <Box position="absolute" right={0}>
          <FormGroup row>
            <FormControlLabel
              control={
                <Switch
                  checked={darkMode}
                  onChange={handleDarkMode}
                  name="DarkMode"
                />
              }
              label="Dark mode"
            />
          </FormGroup>
        </Box>
      </Toolbar>
    </AppBar>
  );
}
