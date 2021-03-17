import React, { ReactNode } from "react";
import { makeStyles, Theme, createStyles } from "@material-ui/core/styles";
import Modal from "@material-ui/core/Modal";
import Backdrop from "@material-ui/core/Backdrop";
import Fade from "@material-ui/core/Fade";

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    modal: {
      display: "flex",
      alignItems: "center",
      justifyContent: "center",
    },
    paper: {
      backgroundColor: theme.palette.background.paper,
      padding: theme.spacing(2, 4, 3),
      outline: 0,
      minHeight: '55vh',
      maxHeight: '85vh',
    },
  })
);

// https://material-ui.com/components/modal/
export default function TransitionsModal({
  open,
  onClose,
  children
}: {
  open: boolean;
  onClose?: (open: boolean) => void;
  children?: ReactNode
}) {
  const classes = useStyles();
  return (
    <Modal
      aria-labelledby="transition-modal-title"
      aria-describedby="transition-modal-description"
      className={classes.modal}
      open={open}
      onClose={onClose}
      closeAfterTransition
      BackdropComponent={Backdrop}
      BackdropProps={{
        timeout: 500,
      }}
    >
      <Fade in={open}>
        <div className={classes.paper} style={{ overflow: 'hidden', height: '100%' }}>
          {children}
        </div>
      </Fade>
    </Modal>
  );
}
