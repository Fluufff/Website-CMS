import { IField } from "@ibs/shared";
import { ReactNode } from "react";

import { FIELD_VIEW_MODE } from "../../fields";

export interface IRenderMultipleProps {
	field: IField;
	children: (index: number) => ReactNode;
	fieldPrefix?: string;
	viewMode: FIELD_VIEW_MODE;
}
