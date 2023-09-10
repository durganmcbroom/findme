import Header from "../components/header.tsx";
import {useEffect, useState} from "react";
import {fetch_stubs, image_url, PostStub, SERVER} from "../fetch.ts";
import {
    Avatar,
    Box, Button,
    Card, CardContent,
    CardHeader,
    CardMedia,
    Grid,
    IconButton,
    List,
    ListItem,
    ListItemText,
    Stack
} from "@mui/material";
import styles from "./app.module.css"
import {prettyMilis} from "../utils.ts";
import {NavLink, useNavigate} from "react-router-dom";

export default () => {
    const [posts, setPosts] = useState<PostStub[]>([])

    useEffect(() => {
        fetch_stubs(10, 10).then((it) => {
            setPosts(it)
            console.log(it)
        })
    }, [])

    return <div>
        <Header/>
        <Box
            display="flex"
            justifyContent="center"
            alignItems="center"
            minHeight="100vh"
        >
            <Stack direction={"column"} spacing={2}>
                {posts.map((stub, i) => {
                    return <PostCard key={i} stub={stub}/>
                })}
            </Stack>
        </Box>
    </div>
}


function PostCard(
    {stub}: {
        stub: PostStub
    }
) {
    const navigate = useNavigate()

    return <div className={styles.card}><Card sx={{
        width: "100%"
    }}>
        <CardHeader
            title={stub.name}
            subheader={prettyMilis(stub.last_seen)}
        />
        <CardMedia
            component="img"
            height="194"
            image={image_url(stub.image)}
            alt={stub.name}

        />

        <CardContent>
            <List>
                <ListItem>
                    <ListItemText
                        primary={"Age:"}
                        secondary={stub.age}
                    />
                    <ListItemText
                        primary={"Issue:"}
                        secondary={stub.disaster_type}
                    />
                </ListItem>
            </List>
            <div className={styles.cardDescription}>
                {stub.description}
            </div>
            <div>
                <NavLink to={`/post/${stub.post_id}`} style={{
                    textDecoration: "none",
                    color: "inherit"
                }}>
                    <Button>
                        See more
                    </Button>
                </NavLink>
            </div>
        </CardContent>
    </Card>
    </div>
}