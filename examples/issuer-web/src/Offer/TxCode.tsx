import Box from '@mui/material/Box';
import Stack from '@mui/material/Stack';
import Typography from '@mui/material/Typography';

export type TxCodeProps = {
    pin: string;
};

const TxCode = (props: TxCodeProps) => {
    const { pin } = props;

    return (
        <Box
            sx={{
                border: 'solid 1px',
                borderRadius: '8px',
                borderColor: theme => theme.palette.primary.light,
                p: 2,
            }}
        >
            <Stack>
                <Typography variant="h5">
                    Transaction Code (PIN)
                </Typography>
                <Typography variant="body2">
                    To complete the transaction, provide this code when prompted by the wallet.
                </Typography>
                <Typography variant="h5">
                    {pin}
                </Typography>
                <Typography variant="fineprint">
                    In a real-world scenario, the transaction code would be securely transmitted to
                    the user on another channel.
                </Typography>
            </Stack>
        </Box>
    );
};

export default TxCode;
