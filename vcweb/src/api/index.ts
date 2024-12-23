import {
    CreateOfferRequest,
    CreateOfferResponse,
    ErrorResponse,
} from '../types/generated';

const stdHeaders = {
    'Content-Type': 'application/json',
};

const svcUrl = import.meta.env.VITE_VERCRE_HTTP_ADDR || 'http://localhost:8080';

// Reusable API request.
export const doRequest = async <Req, Res>(
    endpoint: string,
    req?: Req,
    method?: string,
): Promise<Res> => {
    console.debug(svcUrl, endpoint, req, method);
    let response;
    if (method === 'GET' || method === 'DELETE') {
        let url = `${svcUrl}/${endpoint}`;
        if (req) {
            // CAUTION: This assumes a flat structure with no arrays.
            const params = new URLSearchParams();
            for (const [key, value] of Object.entries(req)) {
                params.append(key, value as string);
            }
            url += `?${params}`;
        }
        response = await fetch(url, {
            method,
            headers: { ...stdHeaders },
        });
    } else {
        response = await fetch(`${svcUrl}/${endpoint}`, {
            method: method || 'POST',
            headers: { ...stdHeaders },
            body: req ? JSON.stringify(req) : undefined,
        });
    }
    if (!response.ok) {
        const msg = `${response.status}: ${response.statusText}`;
        console.error("failed request", { "request_endpoint": endpoint, "message": msg });
        throw Error(msg);
    }
    // We cannot parse JSON directly because an empty response is possible in some cases, but is
    // not valid JSON. So get the response as a string, then parse it if it is not empty.
    const strResult = await response.text();
    const result = JSON.parse(strResult);
    if (result?.error) {
        const err = result as ErrorResponse;
        const msg = err.message;
        console.error("failed request", { "request_endpoint": endpoint, "message": msg });
        throw Error(msg);
    }
    return result as Res;
};

// Create a credential offer.
export const createOffer = async (req: CreateOfferRequest): Promise<CreateOfferResponse> => {
    return doRequest('create_offer', req);
}