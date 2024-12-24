import { useEffect, useState } from "react";

import Box from "@mui/material/Box";
import Button from "@mui/material/Button";
import Grid from "@mui/material/Grid2";
import Stack from "@mui/material/Stack";
import Typography from "@mui/material/Typography";
import { useMutation } from "@tanstack/react-query";
import { useSetRecoilState } from "recoil";

import CreateRequest from "./CreateRequest";
import { instanceOfErrorResponse } from "../api";
import { createRequest } from "../api/verification";
import FullLogo from "../components/FullLogo";
import QrCode from "../components/QrCode";
import { headerState } from "../state";
import { GenerateConstraints, GenerateField, GenerateInputDescriptor, GenerateRequest,
    GenerateRequestResponse } from "../types/generated";

const Request = () => {
    const [processing, setProcessing] = useState<"EmployeeID_JWT" | "Developer_JWT" | null>(null);
    const [qrCode, setQrCode] = useState<string>("");
    const setHeader = useSetRecoilState(headerState);

    useEffect(() => {
        setHeader({
            title: "Credential Verifier",
            action: undefined,
            secondaryAction: undefined,
        });
    }, [setHeader]);

    // Effect to scroll back to top on reset
    useEffect(() => {
        if (processing === null) {
            document.getElementById("pageContent")?.scrollTo({
                top: 0,
                behavior: "smooth",
            });
        }
    }, [processing]);

    // API call to create a presentation request.
    const mut = useMutation({
        mutationFn: async (generateRequest: GenerateRequest) => {
            const response = await createRequest(generateRequest);
            if (instanceOfErrorResponse(response)) {
                console.error(response);
            } else {
                const res = response as GenerateRequestResponse;
                setQrCode(res.qr_code);
            }
        },
        onError: (err) => {
            console.error(err);
        },
        retry: false,
    });

    const handleCreateRequest = async (configId: "EmployeeID_JWT" | "Developer_JWT") => {
        setProcessing(configId);
        const req: GenerateRequest = {
            purpose: purpose(configId),
            // eslint-disable-next-line camelcase
            input_descriptors: inputDescriptors(configId),
        };
        mut.mutate(req);
    };

    return (
        <Box>
            Request
        </Box>
    );
};

export default Request;