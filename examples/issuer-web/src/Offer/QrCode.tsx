import Box from '@mui/material/Box';
import Stack from '@mui/material/Stack';
import Typography from '@mui/material/Typography';

import TxCode from './TxCode';

export type QrCodeProps = {
    image: string;
    pin?: string;
};

const QrCode = (props: QrCodeProps) => {
    const { image, pin } = props;

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
                <Typography variant="body2" gutterBottom>
                    Scan the QR code with a wallet app to view the credential offer.
                </Typography>
                <Box sx={{ display: 'flex', justifyContent: 'center' }}>
                <Box
                    component="img"
                    src={image}
                    alt="QR Code"
                    sx={{
                        maxWidth: 240,
                    }}
                />
                </Box>
                {pin && <TxCode pin={pin} />}
            </Stack>
        </Box>
    );
};

export default QrCode;