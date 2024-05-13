INSERT INTO actix_admin_web.t_schedule_job (name,source,job_type,sys_code,expression,`interval`,`desc`,status,created_at,updated_at) VALUES
	 ('示例任务1',1,1,'task_demo1','',8,'',1,'2024-05-13 21:00:48','2024-05-13 21:00:48'),
	 ('示例任务2',1,0,'task_demo2','1/5 * * * * *',0,'',1,'2024-05-13 21:00:48','2024-05-13 21:00:48');
