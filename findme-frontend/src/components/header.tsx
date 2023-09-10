import styles from "./header.module.css"
import {Button, Stack} from "@mui/material";
import {NavLink} from "react-router-dom";

export default () => {
    return <div id={styles.headerBar}>
        <Stack direction={"row"}>
            <NavLink to={"/"} style={{
                textDecoration: "none",
                color: "inherit"
            }}>
        <span id={styles.headerTitle}>
            <span>Find</span>
            <span>Me</span>
        </span>
            </NavLink>
            <span id={styles.makeReport}>
            <NavLink to={"/register"} style={{
                textDecoration: "none",
                color: "inherit"
            }}>
                <Button variant="outlined">Report someone missing</Button>
            </NavLink>
        </span>
        </Stack>
    </div>
}