import { AnalyticsEventSchema } from "../gen/messages_pb";
import { create, toBinary } from "@bufbuild/protobuf";

const URL = "http://localhost:6690/api/event";

export async function sendAppStartEvent(context, token) {
    const event = create(AnalyticsEventSchema, {
        context,
        eventType: {
            case: "appStart",
            value: {}
        }
    });
    await sendEvent(event, token);
}

export async function sendAppExitEvent(context, token, exitCode) {
    const event = create(AnalyticsEventSchema, {
        context,
        eventType: {
            case: "appExit",
            value: {
                exitCode,
            }
        }
    });
    await sendEvent(event, token);
}

export async function sendUserLoginEvent(context, token, email) {
    const event = create(AnalyticsEventSchema, {
        context,
        eventType: {
            case: "userLogin",
            value: {
                email,
            }
        }
    });
    await sendEvent(event, token);
}

export async function sendUserLogoutEvent(context, token, email) {
    const event = create(AnalyticsEventSchema, {
        context,
        eventType: {
            case: "userLogout",
            value: {
                email,
            }
        }
    });
    await sendEvent(event, token);
}

export async function sendUserRegisterEvent(context, token, email, workspaceId) {
    const event = create(AnalyticsEventSchema, {
        context,
        eventType: {
            case: "userRegister",
            value: {
                email,
                workspaceId,
            }
        }
    });
    await sendEvent(event, token);
}

export async function sendChatCreatedEvent(context, token, workspaceId) {
    const event = create(AnalyticsEventSchema, {
        context,
        eventType: {
            case: "chatCreated",
            value: {
                workspaceId,
            }
        }
    });
    await sendEvent(event, token);
}

export async function sendMessageSentEvent(context, token, chatId, type, size, totalFiles) {
    const event = create(AnalyticsEventSchema, {
        context,
        eventType: {
            case: "messageSent",
            value: {
                chatId,
                type,
                size,
                totalFiles,
            }
        }
    });
    await sendEvent(event, token);
}

export async function sendChatJoinedEvent(context, token, chatId) {
    const event = create(AnalyticsEventSchema, {
        context,
        eventType: {
            case: "chatJoined",
            value: {
                chatId,
            }
        }
    });
    await sendEvent(event, token);
}

export async function sendChatLeftEvent(context, token, chatId) {
    const event = create(AnalyticsEventSchema, {
        context,
        eventType: {
            case: "chatLeft",
            value: {
                chatId,
            }
        }
    });
    await sendEvent(event, token);
}

export async function sendNavigationEvent(context, token, from, to) {
    const event = create(AnalyticsEventSchema, {
        context,
        eventType: {
            case: "navigation",
            value: {
                from,
                to,
            }
        }
    });
    await sendEvent(event, token);
}

async function sendEvent(event, token) {
    console.log("event:", event);
    try {
        const data = toBinary(AnalyticsEventSchema, event);
        // attach token to the url
        let url = URL + "?token=" + token;
        if (navigator.sendBeacon) {
            console.log("sendBeacon");
            navigator.sendBeacon(url, data);
        } else {
            await fetch(url, {
                method: "POST",
                headers: {
                    "Content-Type": "application/protobuf",
                },
                body: data
            });
        }
    } catch (error) {
        console.error("sendEvent error:", error);
    }
}
