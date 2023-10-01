import { Box, Button, Flex, Input, Tag } from "@chakra-ui/react";
import { useState } from "react";
import { BACKEND_URL } from "../config/config";

function ShortenerForm() {
    const [shortenedURL, setShortenedURL] = useState<string | null>(null);
    const [originalURL, setOriginalURL] = useState<string | null>(null);

    function handleClick() {
        if (originalURL === null) {
            return;
        }

        const url = `${BACKEND_URL}/shorten`;
        fetch(url, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(
                {
                    "original_url": originalURL,
                }
            ),
        })
            .then(response => response.json())
            .then(response => {
                setShortenedURL(response);
            })
            .catch(error => {
                console.error(error);
            });
    }

    return (
        <div>
            <Flex direction={"row"}>
                <Box width={"40%"}>
                    <Input
                        placeholder="URL to shorten..."
                        onChange={(event) => setOriginalURL(event.target.value)}
                    />
                </Box>
                <Box width={"40%"}>
                    <Button
                        onClick={() => handleClick()}
                    >Shorten</Button>
                </Box>
            </Flex>
            {
                shortenedURL !== null
                &&
                <Tag>{shortenedURL}</Tag>
            }
        </div>
    );
}

export default ShortenerForm;