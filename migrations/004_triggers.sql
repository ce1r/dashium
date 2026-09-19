CREATE TRIGGER trigger_update_stats
AFTER INSERT OR UPDATE OR DELETE ON completions
FOR EACH ROW
EXECUTE FUNCTION update_stats();

CREATE TRIGGER trigger_update_creator_points
AFTER INSERT OR UPDATE OR DELETE ON rates
FOR EACH ROW
EXECUTE FUNCTION update_creator_points();
