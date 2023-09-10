import Header from "../components/header.tsx";
import styles from "./report.module.css"
import {Box, Button, FormControl, Grid, InputLabel, MenuItem, Select, Stack, TextField} from "@mui/material";
import React, {ChangeEvent, useState} from "react";
import {DisasterType, submit_image, submit_report} from "../fetch.ts";
import {stringDateToMilis, stringDisasterToEnum} from "../utils.ts";
import {useNavigate} from "react-router-dom";


export default () => {
    return <div>
        <Header/>
        <Box
            display="flex"
            justifyContent="center"
            alignItems="center"
            // minHeight="100vh"
        >
            <h1 id={styles.header}>Report a missing person</h1>

            <ReportForm/>

        </Box>
    </div>
}

function ReportForm() {
    const navigate = useNavigate()

    const [formData, setFormData] = useState({
        name: '',
        age: '',
        description: '',
        lastSeen: '',
        location: '',
        disasterType: "",
        image: ""
    });

    const handleChange = (event: React.ChangeEvent<HTMLInputElement | HTMLSelectElement>) => {
        const {name, value} = event.target;
        setFormData({
            ...formData,
            [name]: value,
        });
    };

    const handleImageChange = (event: ChangeEvent<HTMLInputElement>) => {
        let file = event.target.files && event.target.files[0]
        if (file) {
            submit_image(file).then((s) => {
                console.log(s)
                setFormData({
                    ...formData,
                    image: s
                })
            })
        }
    }

    const handleSubmit = (event: React.FormEvent) => {
        event.preventDefault();

        submit_report({
            name: formData.name,
            image: formData.image,
            description: formData.description,
            age: parseInt(formData.age, 10),
            last_seen: stringDateToMilis(formData.lastSeen),
            disaster_type: formData.disasterType,
            last_location: {
                lat: 10,
                lon: 10,
            }
        }).then(() => {
            navigate("/")
        })
    };

    return (
        <div>
            <form onSubmit={handleSubmit} style={{maxWidth: "50vw"}}>
                <Grid container spacing={2}>
                    <Grid item xs={12}>
                        <TextField
                            fullWidth
                            label="Name"
                            name="name"
                            value={formData.name}
                            onChange={handleChange}
                            required
                        />
                    </Grid>
                    <Grid item xs={12}>
                        <TextField
                            fullWidth
                            label="Age"
                            name="age"
                            type="number"
                            value={formData.age}
                            onChange={handleChange}
                            required
                        />
                    </Grid>
                    <Grid item xs={12}>
                        <TextField
                            fullWidth
                            label="Description"
                            name="description"
                            value={formData.description}
                            onChange={handleChange}
                            required
                        />
                    </Grid>
                    <Grid item xs={12}>
                        <TextField
                            fullWidth
                            label="Last Seen"
                            name="lastSeen"
                            type="date"
                            value={formData.lastSeen}
                            onChange={handleChange}
                            required
                        />
                    </Grid>
                    <Grid item xs={12}>
                        <TextField
                            fullWidth
                            label="Location"
                            name="location"
                            value={formData.location}
                            onChange={handleChange}
                            required
                        />
                    </Grid>
                    <Grid item xs={12}>
                        <FormControl fullWidth>
                            <InputLabel>Disaster Type</InputLabel>
                            <Select
                                name="disasterType"
                                value={formData.disasterType}
                                onChange={handleChange}
                                required
                            >
                                {Object.values(DisasterType).map((type) => (
                                    <MenuItem key={type} value={type}>
                                        {type}
                                    </MenuItem>
                                ))}
                            </Select>
                        </FormControl>
                    </Grid>
                    <Grid item xs={12}>
                        <TextField
                            fullWidth
                            type="file"
                            accept="image/*"
                            onChange={handleImageChange}
                        />
                    </Grid>
                    <Grid item xs={12}>
                        <Button type="submit" variant="contained" color="primary">
                            Submit
                        </Button>
                    </Grid>
                </Grid>
            </form>
        </div>
    );
}

