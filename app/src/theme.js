import { createMuiTheme } from "@material-ui/core/styles";

// Colors: https://material.io/resources/color/#!/?view.left=0&view.right=0&primary.color=ffc107&secondary.color=03a9f4
export const getCustomTheme = (darkMode) => {
  return createMuiTheme({
    palette: {
      type: darkMode ? "dark" : "light",
      primary: {
        light: "#fff350",
        main: "#ffc107",
        dark: "#c79100",
        contrastText: "#fff",
      },
      secondary: {
        light: "#67daff",
        main: "#03a9f4",
        dark: "#007ac1",
        contrastText: "#000",
      },
    },
  });
};
