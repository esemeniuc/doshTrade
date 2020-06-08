import React from 'react';
// import {createStyles, makeStyles, Theme} from '@material-ui/core/styles';
// import Drawer from '@material-ui/core/Drawer';
// import List from '@material-ui/core/List';
// import Divider from '@material-ui/core/Divider';
// import ListItem from '@material-ui/core/ListItem';
// import ListItemIcon from '@material-ui/core/ListItemIcon';
// import ListItemText from '@material-ui/core/ListItemText';
// import SettingsIcon from '@material-ui/icons/Settings';
// import {DRAWER_WIDTH} from '../constants';
// import {useHistory} from 'react-router-dom';
//
// const useStyles = makeStyles((theme: Theme) =>
//     createStyles({
//         drawer: {
//             width: DRAWER_WIDTH,
//             flexShrink: 0,
//         },
//         drawerPaper: {
//             width: DRAWER_WIDTH,
//         },
//         // necessary for content to be below app bar
//         toolbar: theme.mixins.toolbar,
//     }),
// );
//
// export default function PermanentDrawerLeft() {
//     const classes = useStyles();
//     const history = useHistory();
//     return (
//         <Drawer
//             className={classes.drawer}
//             variant="permanent"
//             classes={{
//                 paper: classes.drawerPaper,
//             }}
//             anchor="left"
//         >
//             <div className={classes.toolbar} />
//             <Divider />
//             <List>
//                 <ListItem button onClick={() => history.push("/dashboard/manageProperties")}>
//                     <ListItemIcon>
//                         <SettingsIcon />
//                     </ListItemIcon>
//                     <ListItemText primary="Manage Properties" />
//                 </ListItem>
//                 <ListItem button onClick={() => history.push("/dashboard")}>
//                     <ListItemIcon>
//                         <SettingsIcon />
//                     </ListItemIcon>
//                     <ListItemText primary="Dashboard" />
//                 </ListItem>
//             </List>
//         </Drawer>
//     );
// }
