export type RoomStatus = "waiting" | "playing" | "finished";

export interface Room {
    id: string;
    name: string;
    game: string;
    players: {
        current: number;
        max: number;
    };
    status: RoomStatus;
    creator: string;
}

export interface Game {
    id: string;
    name: string;
    description: string;
    icon: string;
    players: string;
    difficulty: string;
}

export const gameTitles: Record<string, string> = {
    tictactoe: "Tic Tac Toe",
    chess: "Chess",
    connect4: "Connect 4",
}; 