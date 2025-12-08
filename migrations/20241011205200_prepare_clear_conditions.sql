ALTER TABLE levels_smm2
  ALTER clear_condition TYPE BIGINT USING (NULLIF(clear_condition, '')::integer),
  ADD clear_condition_magnitude BIGINT;

CREATE INDEX levels_smm2_clear_condition_idx ON levels_smm2 ("clear_condition");
