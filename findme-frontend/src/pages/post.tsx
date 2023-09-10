import {useParams} from "react-router-dom";
import Header from "../components/header.tsx";
import {DisasterType, get_post, image_url, Post, PostCommentType, PostStatus, submit_comment} from "../fetch.ts";
import {useEffect, useState} from "react";
import {
    Box,
    Button,
    Card,
    CardContent,
    CardHeader,
    CardMedia, Input,
    List,
    ListItem,
    ListItemText,
    TextField
} from "@mui/material";
import {prettyMilis} from "../utils.ts";

export default () => {
    const [comment, setComment] = useState("")
    const [commentAuthor, setCommentAuthor] = useState("")
    const {id} = useParams()
    const [post, setPost] = useState<Post>(
        {
            id: 0,
            name: '',
            image: '',
            description: '',
            age: 0,
            last_seen: 0,
            disaster_type: DisasterType.CarCrash,
            last_location: {lat: 0, lon: 0},
            status: PostStatus.NotFound,
            comments: []
        }
    )

    useEffect(() => {
        if (id)
            get_post(parseInt(id)).then((p) => {
                setPost(p)
            })
    }, [])


    return (
        <div>
            <Header/>
            <Box
                display="flex"
                justifyContent="center"
                alignItems="center"
            >
                <Card sx={{
                    width: "80%"
                }}>
                    <CardHeader
                        title={post.name}
                        subheader={prettyMilis(post.last_seen)}
                    />
                    <CardMedia
                        component="img"
                        height="194"
                        image={image_url(post.image)}
                        alt={post.name}
                        onClick={()=>{
                            window.location.assign(image_url(post.image))
                        }}
                    />

                    <CardContent>
                        <List>
                            <ListItem>
                                <ListItemText
                                    primary={"Age:"}
                                    secondary={post.age}
                                />
                                <ListItemText
                                    primary={"Issue:"}
                                    secondary={post.disaster_type}
                                />
                            </ListItem>
                        </List>
                        {post.description}
                        <h2>Comments</h2>
                        <List>
                            {
                                post.comments.map((c, i) => {
                                    return <ListItem key={i} alignItems="flex-start">
                                        <ListItemText
                                            primary={c.author}
                                            secondary={c.value}
                                        />
                                    </ListItem>
                                })
                            }
                        </List>
                        <div>
                            <Input onChange={(event) => {
                                setCommentAuthor(event.target.value)
                            }} sx={{
                                margin: "10px"
                            }} placeholder={"Who are you?"}/>
                        </div>
                        <Input onChange={(event) => {
                            setComment(event.target.value)
                        }} placeholder={"Your comment"} sx={{
                            margin: "10px"
                        }}/>
                        <Button onClick={() => {
                            submit_comment(post.id, {
                                author: commentAuthor,
                                value: comment,
                                status: "Informational"
                            }).then(() => {
                                location.reload()
                            })
                        }}>
                            Comment
                        </Button>
                    </CardContent>
                </Card>
            </Box>
        </div>)
}