import Box from '@mui/material/Box';

export type QrCodeProps = {
    image: string;
};

const QrCode = (props: QrCodeProps) => {
    const { image } = props;

    return (
        <Box
            component="img"
            src={image}
            alt="QR Code"
        />
    );
};

export default QrCode;