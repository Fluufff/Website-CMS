import { FC } from 'react';
import { Controller, useFormContext } from 'react-hook-form';
import cx from 'classnames/bind';
import { Tooltip } from 'react-tooltip';
import classNames from 'classnames';

import { FieldViewMode, IRenderControllerField } from '../fields.types';
import { FieldLabel } from '../../field-helpers/field-label/field-label.component';
import { FieldValue } from '../../field-helpers/field-value/field-value.component';
import { Select } from '../../components';
import { FieldDiff } from '../../field-helpers/field-diff/field-diff.component';

import { ISelectFieldProps, ISelectOptions } from './select-field.types';
import styles from './select-field.module.scss';

const cxBind = cx.bind(styles);

export const SelectField: FC<ISelectFieldProps> = ({ name, label, viewMode = FieldViewMode.EDIT, fieldConfiguration, field, disabled }: ISelectFieldProps) => {
	const { control } = useFormContext();

	const getMappedValue = (value: string | string[]): ISelectOptions | ISelectOptions[] | undefined => {
		const flattenedFields = ((fieldConfiguration?.options as ISelectOptions[]) || []).reduce((acc, option: ISelectOptions) => {
			if (option.options) {
				return [...acc, ...option.options];
			}

			return [...acc, option];
		}, [] as ISelectOptions[]);
		
		if (Array.isArray(value)) {
			return flattenedFields.filter(({ value: optionValue }) => value.includes(optionValue));
		}

		return flattenedFields.find(({ value: optionValue }) => optionValue === value);
	};

	const renderField = ({ field: { onChange, value }, formState: { errors } }: IRenderControllerField) => {
		const mappedValue = getMappedValue(value);

		return (
			<div
				className={cxBind('a-input', {
					'a-input--has-error': !!errors?.[name],
				})}
			>
				{label && (
					<label htmlFor={name} className={cxBind('a-input__label')}>
						{label}
					</label>
				)}
				<div className={classNames(cxBind('a-input__field-wrapper'), 'a-input__field-wrapper')}>
					<Select
						disabled={disabled}
						min={field?.min ?? 1}
						max={field?.max ?? 1}
						hasError={!!errors?.[name]}
						onChange={onChange}
						value={mappedValue}
						options={(fieldConfiguration?.options as ISelectOptions[]) || []}
					/>
					{errors?.[name] && (
						<>
							<Tooltip anchorSelect={`#${name}-err-tooltip`}>{errors?.[name]?.message?.toString()}</Tooltip>
							<div className={cxBind('a-input__error')} id={`${name}-err-tooltip`}>
								<i className="las la-exclamation-triangle"></i>
							</div>
						</>
					)}
				</div>
			</div>
		);
	};

	const renderValue = () => (
		<div className={cxBind('a-input')}>
			<FieldLabel label={label} multiLanguage={fieldConfiguration?.multiLanguage as boolean} viewMode={viewMode} name={name} />
			<FieldValue name={name} />
		</div>
	)

	const renderDiff = () => (
		<div className={cxBind('a-input')}>
			<FieldLabel label={label} multiLanguage={fieldConfiguration?.multiLanguage as boolean} viewMode={viewMode} name={name} />
			<FieldDiff name={name} />
		</div>
	)

	return (
		<div className={cxBind('a-input__field-wrapper')}>
			{viewMode === FieldViewMode.EDIT && <Controller control={control} name={name} render={renderField} />}
			{viewMode === FieldViewMode.VIEW && renderValue()}
			{viewMode === FieldViewMode.DIFF && renderDiff()}
		</div>
	);
};
