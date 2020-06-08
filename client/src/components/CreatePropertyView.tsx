import React, {useState} from 'react';
import {Box, Button, Container, TextField, Typography} from "@material-ui/core";
import {gql, useMutation} from "@apollo/client";
import {CreatePropertyMutation} from "./__generated__/CreatePropertyMutation";
import {loader} from "graphql.macro";

const CREATEPROPERTY_MUTATION = gql`
    mutation CreatePropertyMutation($websiteName: String!, $websiteUrl: String!) {
        createProperty(websiteName: $websiteName, websiteUrl: $websiteUrl) {
            id
        }
    }
`;

//
// function PropertyList() {
//     const {loading, error, data} = useQuery<GetPropertiesQuery>(GETPROPERTIES_QUERY);
//     if (loading) return <>Loading!</>;
//     if (error) return <>{`Error! ${error}`}</>;
//     if (!data) return <>Error! no data</>;
//
//     return <TableContainer component={Paper}>
//         <Table>
//             <TableHead>
//                 <TableRow>
//                     <TableCell>Property ID</TableCell>
//                     <TableCell>Name</TableCell>
//                     <TableCell>URL</TableCell>
//                 </TableRow>
//             </TableHead>
//             <TableBody>
//                 {
//                     data.getProperties.map((e, idx) => <TableRow key={idx}>
//                             <TableCell>{e.id}</TableCell>
//                             <TableCell>{e.websiteName}</TableCell>
//                             <TableCell>{e.websiteUrl}</TableCell>
//                         </TableRow>
//                     )
//                 }
//             </TableBody>
//         </Table>
//     </TableContainer>;
// }
const GETPROPERTIES_QUERY = loader('../graphql/getProperties.gql');

export default function CreatePropertyView() {
    const [websiteName, setWebsiteName] = useState("");
    const [websiteUrl, setWebsiteUrl] = useState("");
    const [createProperty] = useMutation<CreatePropertyMutation>(
        CREATEPROPERTY_MUTATION, {
            refetchQueries: [{query: GETPROPERTIES_QUERY}],
            awaitRefetchQueries: true,
        }
    );
    const onSubmit = () => {
        createProperty({variables: {websiteName, websiteUrl}});
        setWebsiteName("");
        setWebsiteUrl("");
    };

    return <Container maxWidth="sm">
        <Box p={3}>
            <Typography component="h1" variant="h4" gutterBottom>
                Create New Property
            </Typography>
            <Typography component="p" variant="body1" gutterBottom>
                A property is a website you own and want to see stats for. A Property ID will need to be added to your site to track users.
            </Typography>
            <form onSubmit={e => {
                e.preventDefault(); //stop enter key from refreshing the page
                onSubmit();
            }}>
                <TextField required
                           autoFocus
                           type="text"
                           placeholder="My personal blog"
                           label="Property Name"
                           value={websiteName}
                           onChange={(e) => setWebsiteName(e.target.value)}
                />

                <br/>

                <TextField required
                           type="url"
                           placeholder="https://mysite.com/"
                           label="URL"
                           value={websiteUrl}
                           onChange={(e) => setWebsiteUrl(e.target.value)}
                />

                <Box p={2}/>

                <Button variant="contained"
                        color="primary"
                        type="submit"
                        onClick={onSubmit}>
                    Submit
                </Button>
            </form>
        </Box>
    </Container>;
}