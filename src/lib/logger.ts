import { Logtail } from "@logtail/browser";

const LOGTAIL_SOURCE_TOKEN = import.meta.env
    .VITE_LOGTAIL_SOURCE_TOKEN as string;

const logtail = new Logtail(LOGTAIL_SOURCE_TOKEN);

export default function log($message: string | object) {
    logtail.warn("Something is not quite right.", {
        user: {
            username: "John Doe",
            email: "john@example.com"
        },
        additional_info: {
            tried_accessing: "/url/of/error"
        }
    });

    console.log($message);
    if (typeof $message === "object") {
        logtail.info(JSON.stringify($message));
    }
}
