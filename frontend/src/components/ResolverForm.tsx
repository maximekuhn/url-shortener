import { Box, Button, Flex, Input, Tag } from "@chakra-ui/react";
import { useState } from "react";

function ResolverForm() {
    const [shortenedURL, setShortenedURL] = useState<string | null>(null);
    const [originalURL, setOriginalURL] = useState<string | null>(null);

    function handleClick() {
        if(shortenedURL === null || shortenedURL?.trim() === "") {
            return;
        }

        const url = `http://localhost:9090/resolve/${shortenedURL}`;
        fetch(url, {
            method: "GET",
        })
            .then(response => response.json())
            .then(response => {
                setOriginalURL(response);
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
                        placeholder="URL to resolve..."
                        onChange={(event) => setShortenedURL(event.target.value)}
                    />
                </Box>
                <Box width={"40%"}>
                    <Button
                        onClick={() => handleClick()}
                    >Resolve</Button>
                </Box>
            </Flex>
            {
                originalURL !== null
                &&
                <Tag>{originalURL}</Tag>
            }
        </div>
    );
}

export default ResolverForm;