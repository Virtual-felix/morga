import React from "react";
import { Box, Typography, useTheme } from "@material-ui/core";

export default function List() {
  const theme = useTheme();

  return (
    <Box>
      <Box
        px={3}
        display="flex"
        alignItems="center"
        bgcolor={theme.palette.background.paper}
        borderBottom={1}
        height={theme.spacing(6)}
      >
        <Typography color="textPrimary">Item 1</Typography>
      </Box>

      <Box
        px={3}
        display="flex"
        alignItems="center"
        bgcolor={theme.palette.background.paper}
        borderBottom={1}
        height={theme.spacing(6)}
      >
        <Typography color="textPrimary">Item 2</Typography>
      </Box>
      <Box
        px={3}
        display="flex"
        alignItems="center"
        bgcolor={theme.palette.background.paper}
        borderBottom={1}
        height={theme.spacing(6)}
      >
        <Typography color="textPrimary">Item 3</Typography>
      </Box>
      <Box
        px={3}
        display="flex"
        alignItems="center"
        bgcolor={theme.palette.background.paper}
        borderBottom={1}
        height={theme.spacing(6)}
      >
        <Typography color="textPrimary">Item 4</Typography>
      </Box>
      <Box
        px={3}
        display="flex"
        alignItems="center"
        bgcolor={theme.palette.background.paper}
        borderBottom={1}
        height={theme.spacing(6)}
      >
        <Typography color="textPrimary">Item 5</Typography>
      </Box>
    </Box>
  );
}
